# Simple async test

async def simple_async() -> int:
    return 42

async def call_async() -> int:
    result = await simple_async()
    return result