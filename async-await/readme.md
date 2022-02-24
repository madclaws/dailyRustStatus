# Grokking async await

- model is somewhat similar to javascript's async await
- instead of promises, the delayed process is called futures.

## Needed dependencies

- futures
- tokio (one of the most used async await runtimes)

## common stuff in async-await runtimes
- starting the runtime
- Threads spawned on the runtime are green threads (cooperative)
- spawning futures
    use `task::spawn`
- spawning blocking code
    - we should not block the thread that runs the futures, so blocking code
    should be run on another sys thread.
    use `task::spawn_blocking`
    