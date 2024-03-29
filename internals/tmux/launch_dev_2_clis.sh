#!/bin/bash

# initialize
NETWORK_CONFIG="$HOME/.config/saksaha"
if [ -d $NETWORK_CONFIG ]; then
  rm -rf $NETWORK_CONFIG
fi

WALLET_CONFIG="$HOME/.config/saksaha-wallet"
if [ -d $WALLET_CONFIG ]; then
  rm -rf $WALLET_CONFIG
fi

# profiles
PROFILE_1="--cfg-profile dev_local_1"
PROFILE_2="--cfg-profile dev_local_2"

# ci path
SAKSAHA_WORKSPACE="$HOME/work/saksaha/saksaha"
CI="$SAKSAHA_WORKSPACE/ci"

# endpoint to node 2
ENDPOINT_TO_NODE_2="--saksaha-endpoint http://localhost:34419/rpc/v0"


prepare_saksaha_session() {
# |---------------|---------------|
# | 1: node_1     | 2: node_2     |
# |---------------|---------------|
# | 3: wallet_1   | 4: wallet_2   |
# |---------------|---------------|
# | 5: evl_term_1 | 6: evl_term_2 |
# |---------------|---------------|
  tmux new-session -d -s saksaha
  tmux split-window -h
  tmux split-window -v
  tmux split-window -v

  tmux select-pane -t 1
  tmux split-window -v
  tmux split-window -v

  tmux select-layout tiled

  tmux resize-pane -t 5 -U 32
}

run_node_1() {
  tmux select-pane -t 1
  tmux send-keys "${CI} dev ${PROFILE_1}" ENTER
}

run_node_2(){
  tmux select-pane -t 2
  tmux send-keys "${CI} dev ${PROFILE_2}" ENTER
}

run_wallets() {
  tmux select-pane -t 3
  tmux send-keys "${CI} dev_wallet ${PROFILE_1}" ENTER

  tmux select-pane -t 4
  tmux send-keys "${CI} dev_wallet ${PROFILE_2} ${ENDPOINT_TO_NODE_2}" ENTER
}

run_evl_terms() {
  tmux select-pane -t 5
  tmux send-keys "${CI} dev_evl_term ${PROFILE_1}" ENTER

  tmux select-pane -t 6
  tmux send-keys "${CI} dev_evl_term ${PROFILE_2} ${ENDPOINT_TO_NODE_2}" ENTER
}

attach_to_saksaha_session() {
  tmux attach-session -t saksaha
}


prepare_saksaha_session

run_node_1

sleep 0.5

run_node_2

run_wallets
run_evl_terms


attach_to_saksaha_session
