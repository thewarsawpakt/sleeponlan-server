import os

os.mkdir("./payloads")

for to_generate in ["ack", "syn"]:
    with open(f"./payloads/{to_generate}", "wb") as f:
        f.write(os.urandom(2048))
