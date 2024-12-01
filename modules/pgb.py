import requests
import threading
from rich.progress import Progress, BarColumn, DownloadColumn, TextColumn, TimeRemainingColumn, TransferSpeedColumn

class Download:
    def __init__(self, url, dest, description, num_threads=4):
        self.url = url
        self.dest = dest
        self.num_threads = num_threads
        self.download_file(desc=description)

    def download_file(self, desc):
        response = requests.head(self.url)
        total_size = int(response.headers.get('content-length', 0))
        chunk_size = total_size // self.num_threads

        with open(self.dest, 'wb') as file:
            file.truncate(total_size)

        with Progress(
            TextColumn("[progress.description]{task.description}"),
            BarColumn(),
            DownloadColumn(),
            TextColumn("[progress.percentage]{task.percentage:>3.1f}%"),
            TransferSpeedColumn(),
            TimeRemainingColumn()
        ) as progress:
            task_id = progress.add_task(desc, total=total_size)
            threads = []
            for i in range(self.num_threads):
                start = i * chunk_size
                end = start + chunk_size - 1 if i < self.num_threads - 1 else total_size - 1
                thread = threading.Thread(target=self.download_chunk, args=(self.url, start, end, self.dest, progress, task_id))
                threads.append(thread)
                thread.start()

            for thread in threads:
                thread.join()

    @staticmethod
    def download_chunk(url, start, end, dest, progress, task_id):
        headers = {'Range': f'bytes={start}-{end}'}
        response = requests.get(url, headers=headers, stream=True)
        with open(dest, 'r+b') as file:
            file.seek(start)
            for data in response.iter_content(chunk_size=1024):
                file.write(data)
                progress.update(task_id, advance=len(data))
