#include "note_merge_engine.hpp"

#include <cassert>
#include <cstdint>
#include <utility>
#include <vector>

using organologic::BasicNoteMergeEngine;
using organologic::NoteMergeEngine;

void test_first_on_and_last_off()
{
    NoteMergeEngine engine;

    for (int index = 0; index < 5; ++index) {
        assert(engine.process(60, 100, 1) == (index == 0));
    }
    for (int index = 0; index < 5; ++index) {
        assert(engine.process(60, 0, 1) == (index == 4));
    }
}

void test_notes_and_channels_are_independent()
{
    NoteMergeEngine engine;

    assert(engine.process(60, 100, 1));
    assert(engine.process(60, 100, 2));
    assert(engine.process(61, 100, 1));
    assert(!engine.process(60, 80, 1));

    assert(!engine.process(60, 0, 1));
    assert(engine.process(60, 0, 1));
    assert(engine.process(60, 0, 2));
    assert(engine.process(61, 0, 1));
}

void test_unmatched_off_and_invalid_values_are_ignored()
{
    NoteMergeEngine engine;

    assert(!engine.process(60, 0, 1));
    assert(!engine.process(-1, 100, 1));
    assert(!engine.process(128, 100, 1));
    assert(!engine.process(60, -1, 1));
    assert(!engine.process(60, 128, 1));
    assert(!engine.process(60, 100, 0));
    assert(!engine.process(60, 100, 17));
}

void test_counter_saturates_without_wrapping()
{
    BasicNoteMergeEngine<std::uint8_t> engine;

    assert(engine.process(60, 100, 1));
    for (int index = 1; index < 300; ++index) {
        assert(!engine.process(60, 100, 1));
    }
    assert(engine.count(60, 1) == 255);

    for (int index = 0; index < 254; ++index) {
        assert(!engine.process(60, 0, 1));
    }
    assert(engine.process(60, 0, 1));
    assert(engine.count(60, 1) == 0);
}

void test_clear_and_active_note_iteration()
{
    NoteMergeEngine engine;
    std::vector<std::pair<int, int>> active;

    assert(engine.process(72, 90, 16));
    assert(engine.process(48, 90, 3));
    assert(!engine.process(48, 90, 3));

    engine.for_each_active(
        [&active](int pitch, int channel) { active.emplace_back(pitch, channel); });

    assert((active == std::vector<std::pair<int, int>>{{48, 3}, {72, 16}}));

    engine.clear();
    active.clear();
    engine.for_each_active(
        [&active](int pitch, int channel) { active.emplace_back(pitch, channel); });
    assert(active.empty());
}

int main()
{
    test_first_on_and_last_off();
    test_notes_and_channels_are_independent();
    test_unmatched_off_and_invalid_values_are_ignored();
    test_counter_saturates_without_wrapping();
    test_clear_and_active_note_iteration();
}
