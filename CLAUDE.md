# Shapely Project Guide

## Code Style Guidelines
- Follow Rust snake_case for functions/variables, PascalCase for types/traits
- Group imports: std first, external crates second, internal imports last
- Document public APIs with `///` comments (explain behavior, parameters, return values)
  (don't parrot what the code does: explain the why, not the how)
- Use Result<T, E> with descriptive error types that implement std::error::Error
- Provide context in error messages (including position information where relevant)
- Wrap unsafe code in safe abstractions with clear documentation
- Write thorough unit tests for all functionality
- Use explicit panic messages that explain invariant violations
- Document any undefined behavior with clear warnings

## Important instructions

Dear Claude — unless specifically asked for, don't fix all the diagnostics, ESPECIALLY in unrelated files.

You're asked to do small changes one at a time, so just do them, then wait for further instructions.
Don't go off too far on your own. If you get yourself thinking for too long, stop. Just ask for more
context or ask to make the task smaller. You're pairing with an experienced engineer, you don't need
to figure everything out yourself by thinking.

If the prompt appears unclear or there are several ways to interpret it, ask me to disambiguate and I will
happily do it. I will be happier clarifying ahead of time than undoing your stuff.

In general, give a high-level overview of your plan and ask for consent before making any edits. Thanks.

DON'T FIX DIAGNOSTICS BY YOURSELF. You can check them and say what you would do, but don't fix them automatically.

Don't keep going. Ask for the next thing to do.
