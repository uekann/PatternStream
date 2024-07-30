import asyncio
import os


async def tcp_client(host, port):
    reader, writer = await asyncio.open_connection(host, port)

    print(f'Connected to {host}:{port}')

    try:
        while True:
            data = await reader.read(30)  # 100バイトずつ読み取る
            if not data:
                break
            print(f'Received: {data.decode()}')
    except asyncio.CancelledError:
        pass
    finally:
        print('Closing the connection')
        writer.close()
        await writer.wait_closed()

async def main():
    host = os.getenv("STREAM_SERVER_IP", "127.0.0.1")
    port = int(os.getenv("STREAM_SERVER_PORT", 8080)) 
    await tcp_client(host, port)

if __name__ == '__main__':
    asyncio.run(main())
