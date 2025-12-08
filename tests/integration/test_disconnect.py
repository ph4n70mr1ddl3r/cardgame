import subprocess
import time
import sys
import os

def run_disconnect_test():
    server_bin = os.path.abspath("build/src/server/poker_server")
    client_bin = os.path.abspath("build/src/client/poker_client")
    
    if not os.path.exists(server_bin) or not os.path.exists(client_bin):
        print("Binaries not found. Build first.")
        sys.exit(1)

    print("Starting disconnect test...")

    with open("server.log", "w") as s_log, open("client1.log", "w") as c1_log, open("client2.log", "w") as c2_log:
        server = subprocess.Popen([server_bin, "8083"], stdout=s_log, stderr=s_log)
        time.sleep(1)
        
        # Start Bot1 and Bot2
        c1 = subprocess.Popen([client_bin, "localhost", "8083", "Bot1"], stdout=c1_log, stderr=c1_log)
        c2 = subprocess.Popen([client_bin, "localhost", "8083", "Bot2"], stdout=c2_log, stderr=c2_log)
        
        time.sleep(5)
        
        # Kill Bot1
        print("Killing Bot1...")
        c1.terminate()
        c1.wait()
        
        time.sleep(2)
        
        # Restart Bot1 (Reconnect)
        print("Restarting Bot1...")
        c1_reconnect_log = open("client1_reconnect.log", "w")
        c1_new = subprocess.Popen([client_bin, "localhost", "8083", "Bot1"], stdout=c1_reconnect_log, stderr=c1_reconnect_log)
        
        time.sleep(5)
        
        c1_new.terminate()
        c2.terminate()
        server.terminate()
        c1_reconnect_log.close()
        
    # Verify logs
    with open("server.log", "r") as f:
        s_out = f.read()
    
    if "Player reconnected: Bot1" in s_out:
        print("SUCCESS: Reconnection detected.")
    else:
        print("FAILURE: Reconnection NOT detected.")
        print("Server Log Tail:")
        print(s_out[-500:])
        sys.exit(1)

if __name__ == "__main__":
    run_disconnect_test()
