#pragma once

#include <array>
#include <cstdint>
#include <limits>
#include <type_traits>

namespace organologic {

template <typename Counter = std::uint32_t>
class BasicNoteMergeEngine {
    static_assert(std::is_unsigned_v<Counter>, "Counter must be unsigned");

public:
    static constexpr int channel_count = 16;
    static constexpr int note_count = 128;

    [[nodiscard]] bool process(int pitch, int velocity, int channel) noexcept
    {
        if (!valid_note(pitch, velocity, channel)) {
            return false;
        }

        auto& count = counts_[static_cast<std::size_t>(channel - 1)]
                             [static_cast<std::size_t>(pitch)];

        if (velocity > 0) {
            const bool first_note_on = count == 0;
            if (count < (std::numeric_limits<Counter>::max)()) {
                ++count;
            }
            return first_note_on;
        }

        if (count == 0) {
            return false;
        }

        --count;
        return count == 0;
    }

    void clear() noexcept
    {
        counts_ = {};
    }

    template <typename Callback>
    void for_each_active(Callback&& callback) const
    {
        for (std::size_t channel = 0; channel < counts_.size(); ++channel) {
            for (std::size_t pitch = 0; pitch < counts_[channel].size(); ++pitch) {
                if (counts_[channel][pitch] != 0) {
                    callback(static_cast<int>(pitch), static_cast<int>(channel + 1));
                }
            }
        }
    }

    [[nodiscard]] Counter count(int pitch, int channel) const noexcept
    {
        if (pitch < 0 || pitch >= note_count || channel < 1 || channel > channel_count) {
            return 0;
        }
        return counts_[static_cast<std::size_t>(channel - 1)]
                      [static_cast<std::size_t>(pitch)];
    }

private:
    [[nodiscard]] static bool valid_note(int pitch, int velocity, int channel) noexcept
    {
        return pitch >= 0 && pitch < note_count && velocity >= 0 && velocity < 128
            && channel >= 1 && channel <= channel_count;
    }

    std::array<std::array<Counter, note_count>, channel_count> counts_{};
};

using NoteMergeEngine = BasicNoteMergeEngine<>;

} // namespace organologic
