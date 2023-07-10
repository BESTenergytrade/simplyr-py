import simplyr_py

import json
import random


if __name__ == "__main__":
    orders = []

    for i in range(10):
        orders.append({
            'id': i,
            "order_type": random.choice(["bid", "ask"]),
            "time_slot":"2022-03-04T05:06:07+00:00",
            "actor_id": f"actor_{i}",
            "cluster_index": 0,
            "energy_kwh": round(random.random()*10, 1),
            "price_euro_per_kwh": round(random.random() * 0.3 + 0.1, 2)
        })

    #print(json.dumps(orders, indent=2))

    results = simplyr_py.run("", json.dumps(orders))

    print(results)
    print("TERMINATE")
