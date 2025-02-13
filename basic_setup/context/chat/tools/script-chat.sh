#!/bin/bash

# Node configurations
NODE1="chatNode1"
NODE2="chatNode2"

# Executor IDs for each node
EXECUTOR_ID_NODE1="3DZLhK1rWUj5Bxtio3txyHXgf8n2Lu8HczRxJuzZUMWF"
EXECUTOR_ID_NODE2="3c9jzEZWH1rjagH5Cj8JfQs5oSvenooiLH6yUAgUtwX8"

# Context ID shared between nodes
CONTEXT_ID="D9iUBk7pZLR3h8D5SLQe4kHxgkFA7NM4pdwjEPmLVcvd"

# Select which node to use (NODE1 or NODE2)
SELECTED_NODE=$NODE2
# Get the corresponding executor ID
EXECUTOR_ID=""
if [ "$SELECTED_NODE" = "$NODE1" ]; then
    EXECUTOR_ID=$EXECUTOR_ID_NODE1
else
    EXECUTOR_ID=$EXECUTOR_ID_NODE2
fi

# Function to get messages
get_messages() {
    local node=$1
    local executor_id=$2
    echo "Getting messages from $node..."
    meroctl --node-name "$node" call --as "$executor_id" "$CONTEXT_ID" get_all_messages
}

# Function to send message
send_message() {
    local node=$1
    local executor_id=$2
    local sender=$3
    local message=$4
    echo "Sending message from $sender using $node..."
    meroctl --node-name "$node" call --as "$executor_id" "$CONTEXT_ID" send_message --args "{\"sender\": \"$sender\", \"content\": \"$message\"}"
}

# Usage examples:
echo "Available commands:"
echo "1. Get messages from node1: get_messages \$NODE1 \$EXECUTOR_ID_NODE1"
echo "2. Get messages from node2: get_messages \$NODE2 \$EXECUTOR_ID_NODE2"
echo "3. Send message from node1: send_message \$NODE1 \$EXECUTOR_ID_NODE1 \"Peter\" \"your message\""
echo "4. Send message from node2: send_message \$NODE2 \$EXECUTOR_ID_NODE2 \"Peter\" \"your message\""

# To send a message from Peter using node1:
send_message $NODE1 $EXECUTOR_ID_NODE1 "Peter" "Hello from Peter"

# To send a message from Maria using node2:
send_message $NODE2 $EXECUTOR_ID_NODE2 "Maria" "Hello from Maria"

# To check messages:
get_messages $NODE1 $EXECUTOR_ID_NODE1