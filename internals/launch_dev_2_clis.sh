#!/bin/bash
tmux new-session -d -s saksaha
tmux split-window -h
tmux split-window -v
tmux split-window -v

tmux select-pane -t 1
tmux split-window -v
tmux split-window -v

tmux select-layout tiled

tmux resize-pane -t 1 -U 15
tmux resize-pane -t 2 -U 15
# 6 panes have been displayed

# |---------------|---------------|
# | 1: node_1     | 2: node_2     |
# |---------------|---------------|
# | 3: wallet_1   | 4: wallet_2   |
# |---------------|---------------|
# | 5: evl_term_1 | 6: evl_term_2 |
# |---------------|---------------|


ci="~/work/saksaha/saksaha/ci"

clear_node_1="rm -rf ~/.config/saksaha/dev_local_1/"
clear_node_2="rm -rf ~/.config/saksaha/dev_local_2/"

profile_1="--cfg-profile dev_local_1"
profile_2="--cfg-profile dev_local_2"

run_node_1="${ci} dev ${profile_1}"
run_node_2="${ci} dev ${profile_2}"

clear_wallet="rm -rf ~/.config/saksaha-wallet"

run_wallet_1="${ci} dev_wallet ${profile_1}"
run_wallet_2="${ci} dev_wallet ${profile_2}"

endpoint_to_node_2="--saksaha-endpoint http://localhost:34419/rpc/v0"

run_evl_term_1="${ci} dev_evl_term ${profile_1}"
run_evl_term_2="${ci} dev_evl_term ${profile_2}"


# node
tmux select-pane -t 1
tmux send-keys "${clear_node_1}" ENTER
tmux send-keys "${run_node_1}" ENTER

tmux select-pane -t 2
tmux send-keys "${clear_node_2}" ENTER
tmux send-keys "${run_node_2}" ENTER

# # wallet
tmux select-pane -t 3
tmux send-keys "${clear_wallet}" ENTER
tmux send-keys "${run_wallet_1}" ENTER

tmux select-pane -t 4
tmux send-keys "${run_wallet_2} ${endpoint_to_node_2}" ENTER

# # term
tmux select-pane -t 5
tmux send-keys "${run_evl_term_1}" ENTER

tmux select-pane -t 6
tmux send-keys "${run_evl_term_2} ${endpoint_to_node_2}" ENTER

tmux attach-session -t saksaha
