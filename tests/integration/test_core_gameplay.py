import subprocess
import time
import sys
import os

def run_integration_test():
    server_bin = os.path.abspath("build/src/server/poker_server")
    client_bin = os.path.abspath("build/src/client/poker_client")
    
    if not os.path.exists(server_bin) or not os.path.exists(client_bin):
        print("Binaries not found. Build first.")
        sys.exit(1)

    print("Starting integration test...")

    with open("server.log", "w") as s_log, open("client1.log", "w") as c1_log, open("client2.log", "w") as c2_log:
        server = subprocess.Popen([server_bin, "8082"], stdout=s_log, stderr=s_log)
        time.sleep(1)
        
        c1 = subprocess.Popen([client_bin, "localhost", "8082", "Bot1"], stdout=c1_log, stderr=c1_log)
        time.sleep(0.5)
        c2 = subprocess.Popen([client_bin, "localhost", "8082", "Bot2"], stdout=c2_log, stderr=c2_log)
        
        print("Running for 10 seconds...")
        time.sleep(10)
        
        c1.terminate()
        c2.terminate()
        server.terminate()
        
    # Verify logs
    with open("server.log", "r") as f:
        s_out = f.read()
    
    if "Starting game..." in s_out:
        print("SUCCESS: Game started.")
    else:
        print("FAILURE: Game did not start.")
        print("Server Log Tail:")
        print(s_out[-500:])
        sys.exit(1)

    if "Action from Bot1" in s_out or "Action from Bot2" in s_out:
        print("SUCCESS: Actions received.")
    else:
        print("FAILURE: No actions received.")
        sys.exit(1)

if __name__ == "__main__":
    run_integration_test()
