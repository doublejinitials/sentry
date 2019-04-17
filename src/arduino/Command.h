#ifndef SENTRY_COMMON_COMMAND_H
#define SENTRY_COMMON_COMMAND_H

namespace Sentry {
namespace Common {

typedef enum : uint8_t {
    COMMAND_NONE = 0,
    COMMAND_HOME,
    COMMAND_OPEN_BREACH,
    COMMAND_CLOSE_BREACH,
    COMMAND_CYCLE_BREACH,
    COMMAND_FIRE,
    COMMAND_FIRE_AND_CYCLE_BREACH,
} Command;

} // namespace Common
} // namespace Sentry

#endif // SENTRY_COMMON_COMMAND_H