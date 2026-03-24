import redis
import time
from django.http import JsonResponse

r = redis.Redis(host='redis', port=6379, db=0)
# the humble yet elegant bubble sort

def bubble_sort_view(request):
    data = list(range(100000, 0, -1))  # 100k reversed integers

    n = len(data)
    aborted = False
    start_time = time.time()

    total_swaps = 0
    total_comparisons = 0

    for i in range(n):
        if i % 50 == 0:
            progress = int((i / n) * 100)
            elapsed_ms = int((time.time() - start_time) * 1000)

            r.set("django_progress", progress)
            r.set("django_swaps", total_swaps)
            r.set("django_comparisons", total_comparisons)
            r.set("django_elapsed_ms", elapsed_ms)

            if r.get("race_winner") == b"rust":
                aborted = True
                break

        for j in range(0, n - i - 1):
            total_comparisons += 1
            if data[j] > data[j + 1]:
                data[j], data[j + 1] = data[j + 1], data[j]
                total_swaps += 1

    duration = int((time.time() - start_time) * 1000)
    status = "KILLED BY RUST" if aborted else "COMPLETED"

    if not aborted:
        r.set("django_progress", 100)
        r.set("django_swaps", total_swaps)
        r.set("django_comparisons", total_comparisons)
        r.set("django_elapsed_ms", duration)
        r.setnx("race_winner", "django")

    return JsonResponse({
        "engine": "python/django",
        "status": status,
        "time_ms": duration,
        "swaps": total_swaps,
        "comparisons": total_comparisons,
        "iterations": i,
    })