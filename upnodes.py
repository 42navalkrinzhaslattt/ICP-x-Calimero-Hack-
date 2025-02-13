#!/usr/bin/env python3
"""
Automate merod commands in a single interactive session using pexpect.

This script:
  1. Spawns the node with 'merod --node-name node1 run' in a pseudo terminal.
  2. Waits for the node to initialize.
  3. Sends the 'context ls' command to retrieve context information.
  4. Parses the output to extract the Context ID.
  5. Similarly sends the 'identity ls <Context ID>' command.
  6. Then issues the remaining commands (set_seed, process_value, get_last_value)
     within the same session.
"""

import pexpect
import time
import re
import logging
import sys

# Configure logging for clarity.
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

def spawn_node():
    # Start the node process in an interactive pseudo terminal.
    logging.info("Spawning merod node in interactive session...")
    child = pexpect.spawn("merod --node-name node1 run", encoding='utf-8', timeout=10)
    child.logfile = sys.stdout  # Direct child output to stdout for debugging.
    return child

def send_command(child, command, expect_prompt=True, wait=1):
    logging.info("Sending command: %s", command)
    child.sendline(command)
    time.sleep(wait)  # Give some time for output to accumulate.
    # Optionally, if the node prints a prompt or specific output, you can use child.expect()
    return child.before  # Return the output before the next prompt.

def extract_first_token(output):
    # Assumes output lines have the desired token on the second line.
    lines = output.splitlines()
    if len(lines) < 2:
        logging.error("Unexpected output format:\n%s", output)
        sys.exit(1)
    # Extract the first token from the second line.
    token = lines[1].strip().split()[0]
    return token

def main():
    child = spawn_node()
    
    # Wait for initialization messages.
    time.sleep(5)
    
    # Step 3: Run "context ls" in the same session.
    context_output = send_command(child, "context ls")
    logging.info("Output of 'context ls':\n%s", context_output)
    
    # Parse the context ID from the output.
    try:
        context_id = extract_first_token(context_output)
        logging.info("Extracted Context ID: %s", context_id)
    except Exception as e:
        logging.error("Failed to parse Context ID: %s", e)
        child.terminate(force=True)
        sys.exit(1)
    
    # Step 4: Run "identity ls <Context ID>"
    identity_cmd = f"identity ls {context_id}"
    identity_output = send_command(child, identity_cmd)
    logging.info("Output of '%s':\n%s", identity_cmd, identity_output)
    
    try:
        identity = extract_first_token(identity_output)
        logging.info("Extracted Identity: %s", identity)
    except Exception as e:
        logging.error("Failed to parse Identity: %s", e)
        child.terminate(force=True)
        sys.exit(1)
    
    # Step 5: Call set_seed
    set_seed_cmd = f'meroctl --node-name node1 call --as {identity} {context_id} set_seed --args \'{{"quantum_seed": [10,20,30,40]}}\''
    set_seed_output = send_command(child, set_seed_cmd)
    logging.info("Output of set_seed:\n%s", set_seed_output)
    
    # Step 6: Call process_value
    process_cmd = f'meroctl --node-name node1 call --as {identity} {context_id} process_value --args \'{{"node": "node1"}}\''
    process_output = send_command(child, process_cmd)
    logging.info("Output of process_value:\n%s", process_output)
    
    # Step 7: Retrieve last generated value.
    get_last_cmd = f'meroctl --node-name node1 call --as {identity} {context_id} get_last_value'
    get_last_output = send_command(child, get_last_cmd)
    logging.info("Output of get_last_value:\n%s", get_last_output)
    
    # Cleanup: terminate the interactive session gracefully.
    logging.info("Terminating interactive session...")
    child.sendline("exit")  # Or use child.terminate() if appropriate.
    child.wait()  # Wait for the process to exit.

if __name__ == "__main__":
    main()
