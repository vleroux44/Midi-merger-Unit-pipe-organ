#include "ext.h"
#include "ext_obex.h"

#include "note_merge_engine.hpp"

#include <array>
#include <new>

namespace {

struct OrganologicMerge {
    t_object object;
    void* velocity_proxy;
    void* channel_proxy;
    void* pitch_outlet;
    void* velocity_outlet;
    void* channel_outlet;
    t_atom_long velocity;
    t_atom_long channel;
    organologic::NoteMergeEngine engine;
};

t_class* organologic_merge_class = nullptr;

void output_note(
    OrganologicMerge* x, t_atom_long pitch, t_atom_long velocity, t_atom_long channel)
{
    // Max propagates messages synchronously. Send stored values first and pitch last,
    // because pitch triggers noteout.
    outlet_int(x->channel_outlet, channel);
    outlet_int(x->velocity_outlet, velocity);
    outlet_int(x->pitch_outlet, pitch);
}

void organologic_merge_int(OrganologicMerge* x, t_atom_long value)
{
    switch (proxy_getinlet(reinterpret_cast<t_object*>(x))) {
        case 2:
            x->channel = value;
            return;
        case 1:
            x->velocity = value;
            return;
        default:
            break;
    }

    if (x->engine.process(static_cast<int>(value), static_cast<int>(x->velocity),
                          static_cast<int>(x->channel))) {
        output_note(x, value, x->velocity, x->channel);
    }
}

void organologic_merge_clear(OrganologicMerge* x)
{
    x->engine.clear();
}

void organologic_merge_panic(OrganologicMerge* x)
{
    std::array<std::array<bool, organologic::NoteMergeEngine::note_count>,
               organologic::NoteMergeEngine::channel_count>
        active{};

    x->engine.for_each_active([&active](int pitch, int channel) {
        active[static_cast<std::size_t>(channel - 1)][static_cast<std::size_t>(pitch)] = true;
    });
    x->engine.clear();

    for (std::size_t channel = 0; channel < active.size(); ++channel) {
        for (std::size_t pitch = 0; pitch < active[channel].size(); ++pitch) {
            if (active[channel][pitch]) {
                output_note(x, static_cast<t_atom_long>(pitch), 0,
                            static_cast<t_atom_long>(channel + 1));
            }
        }
    }
}

void organologic_merge_inletinfo(OrganologicMerge*, void*, long index, char* non_triggering)
{
    *non_triggering = index == 0 ? 0 : 1;
}

void organologic_merge_assist(
    OrganologicMerge*, void*, long message, long index, char* destination)
{
    if (message == ASSIST_INLET) {
        switch (index) {
            case 0:
                snprintf_zero(destination, 256, "Pitch (int, déclenche le traitement), clear ou panic");
                break;
            case 1:
                snprintf_zero(destination, 256, "Vélocité (0 = Note Off)");
                break;
            case 2:
                snprintf_zero(destination, 256, "Canal MIDI (1–16)");
                break;
            default:
                break;
        }
    }
    else {
        switch (index) {
            case 0:
                snprintf_zero(destination, 256, "Pitch filtré");
                break;
            case 1:
                snprintf_zero(destination, 256, "Vélocité filtrée");
                break;
            case 2:
                snprintf_zero(destination, 256, "Canal MIDI filtré");
                break;
            default:
                break;
        }
    }
}

void organologic_merge_free(OrganologicMerge* x)
{
    object_free(x->velocity_proxy);
    object_free(x->channel_proxy);
}

void* organologic_merge_new()
{
    auto* x = static_cast<OrganologicMerge*>(object_alloc(organologic_merge_class));
    if (x == nullptr) {
        return nullptr;
    }

    new (&x->engine) organologic::NoteMergeEngine();
    x->velocity = 0;
    x->channel = 1;

    // Proxies are created from right to left.
    x->channel_proxy = proxy_new(x, 2, nullptr);
    x->velocity_proxy = proxy_new(x, 1, nullptr);

    // Outlets are also created from right to left.
    x->channel_outlet = intout(x);
    x->velocity_outlet = intout(x);
    x->pitch_outlet = intout(x);

    return x;
}

} // namespace

extern "C" C74_EXPORT void ext_main(void*)
{
    auto* c = class_new("organologic.merge", reinterpret_cast<method>(organologic_merge_new),
                        reinterpret_cast<method>(organologic_merge_free), sizeof(OrganologicMerge),
                        nullptr, 0);

    class_addmethod(c, reinterpret_cast<method>(organologic_merge_int), "int", A_LONG, 0);
    class_addmethod(c, reinterpret_cast<method>(organologic_merge_clear), "clear", 0);
    class_addmethod(c, reinterpret_cast<method>(organologic_merge_panic), "panic", 0);
    class_addmethod(c, reinterpret_cast<method>(organologic_merge_assist), "assist", A_CANT, 0);
    class_addmethod(c, reinterpret_cast<method>(organologic_merge_inletinfo), "inletinfo", A_CANT,
                    0);

    class_register(CLASS_BOX, c);
    organologic_merge_class = c;
}
