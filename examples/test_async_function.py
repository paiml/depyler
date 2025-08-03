# Test async function support

async def fetch_data(url: str) -> str:
    # Simulated async operation
    await async_sleep(1)
    return f"Data from {url}"

async def process_urls(urls: list[str]) -> list[str]:
    results = []
    for url in urls:
        data = await fetch_data(url)
        results.append(data)
    return results

async def async_sleep(seconds: int) -> None:
    # Placeholder for async sleep
    pass

async def main() -> None:
    urls = ["http://api.example.com", "http://api2.example.com"]
    results = await process_urls(urls)
    for result in results:
        print(result)