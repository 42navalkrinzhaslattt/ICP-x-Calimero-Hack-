#!/usr/bin/env python3
import pexpect
import time
import re
import sys
import os
import signal

def kill_previous_processes():
    """Убиваем предыдущие процессы merod"""
    try:
        # Ищем PID процесса merod
        pid = int(os.popen("pgrep -f 'merod --node-name node1'").read().strip())
        os.kill(pid, signal.SIGTERM)
        print(f"Killed previous merod process with PID {pid}")
        time.sleep(2)  # Даем время для освобождения портов
    except Exception as e:
        pass  # Если процессов не найдено, игнорируем ошибку

def get_context_id():
    """Получаем Context ID через meroctl"""
    try:
        output = pexpect.run("meroctl --node-name node1 context ls", encoding="utf-8", timeout=10)
        match = re.search(r'^([A-Za-z0-9]{43,})\s*\|', output, re.MULTILINE)
        return match.group(1) if match else None
    except Exception as e:
        print(f"Error getting context: {str(e)}")
        return None

def get_identity_id(context_id):
    """Получаем Identity ID через meroctl"""
    try:
        output = pexpect.run(f"meroctl --node-name node1 identity ls {context_id}", 
                           encoding="utf-8", timeout=10)
        match = re.search(r'^([A-Za-z0-9]{43,})\s*\|.*Yes', output, re.MULTILINE)
        return match.group(1) if match else None
    except Exception as e:
        print(f"Error getting identity: {str(e)}")
        return None

def main():
    # Убиваем предыдущие процессы
    kill_previous_processes()
    
    # 1. Инициализация ноды
    print("=== 1) Node initialization ===")
    try:
        init_output = pexpect.run(
            "merod --node-name node1 init --server-port 2427 --swarm-port 2527",
            encoding="utf-8",
            timeout=30
        )
        print(init_output)
    except pexpect.exceptions.ExceptionPexpect as e:
        print(f"Init error: {str(e)}")
        sys.exit(1)

    # 2. Запуск ноды в фоновом режиме
    print("=== 2) Starting node in background ===")
    try:
        merod_process = pexpect.spawn(
            "merod --node-name node1 run",
            encoding="utf-8",
            timeout=30
        )
        merod_process.expect("INFO libp2p_swarm: local_peer_id=", timeout=15)
    except Exception as e:
        print(f"Failed to start node: {str(e)}")
        sys.exit(1)

    # 3. Получение Context ID
    print("=== 3) Getting Context ID ===")
    context_id = None
    for _ in range(5):  # Повторяем попытки
        context_id = get_context_id()
        if context_id:
            break
        time.sleep(2)
    
    if not context_id:
        print("Failed to get Context ID after 5 attempts")
        merod_process.terminate()
        sys.exit(1)
    print(f"Context ID: {context_id}")

    # 4. Получение Identity ID
    print("=== 4) Getting Identity ID ===")
    identity_id = None
    for _ in range(5):
        identity_id = get_identity_id(context_id)
        if identity_id:
            break
        time.sleep(2)
    
    if not identity_id:
        print("Failed to get Identity ID after 5 attempts")
        merod_process.terminate()
        sys.exit(1)
    print(f"Identity ID: {identity_id}")

    # 5. Выполнение команд через meroctl
    print("=== 5) Executing commands ===")
    try:
        # Set seed
        pexpect.run(
            f'meroctl --node-name node1 call --as {identity_id} {context_id} '
            'set_seed --args \'{{"quantum_seed": [10,20,30,40]}}\'',
            encoding="utf-8",
            timeout=15
        )
        
        # Process values
        pexpect.run(
            f'meroctl --node-name node1 call --as {identity_id} {context_id} '
            'process_value --args \'{{"node": "node1"}}\'',
            encoding="utf-8",
            timeout=15
        )
        
        # Get last value
        output = pexpect.run(
            f'meroctl --node-name node1 call --as {identity_id} {context_id} get_last_value',
            encoding="utf-8",
            timeout=15
        )
        print("Last value:", output)
    
    except Exception as e:
        print(f"Command execution failed: {str(e)}")
    
    # 6. Завершение работы
    print("=== 6) Shutting down ===")
    merod_process.terminate()
    merod_process.wait()

if __name__ == "__main__":
    main()