NETWORK=$1

# Define constants for color-coded output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

# Validate the input arguments
if [ "$#" -ne 1 ]; then
    echo -e "${RED}Error: Invalid number of arguments.${NC}"
    echo -e "${YELLOW}Usage: bash /workspace/scripts/populate_network.sh <network> ${NC}"
    exit 1
fi

case "$1" in
standalone)
  echo -e "Populating standalone network "
  ;;
futurenet)
  echo -e "Populating Futurenet network "
  ;;
testnet)
  echo -e "Populating Futurenet network "
  ;;
testnet-public)
  echo -e "Populating Futurenet network with public RPC https://soroban-testnet.stellar.org/ "
  ;;
*)
  echo -e "${YELLOW}Usage: $0 standalone|futurenet|testnet|testnet-public ${NC}"
  exit 1
  ;;
esac

#Step 1: Deploy 8 tokens and pairs
echo -e "${GREEN}Deploying tokens...${NC}"
bash scripts/deploy_tokens_n_pairs.sh $NETWORK 8

#Step 2: Add Liquidity to multiple pairs
echo -e "${GREEN}Adding liquidity to multiple pairs...${NC}"
bash /workspace/scripts/multi_add_liquidity.sh $NETWORK