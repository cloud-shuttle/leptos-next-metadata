#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Starting Metadata Testing with Test Server${NC}"

# Function to cleanup on exit
cleanup() {
    echo -e "\n${YELLOW}🧹 Cleaning up...${NC}"

    # Kill any running test server
    if [ ! -z "$SERVER_PID" ]; then
        echo -e "${YELLOW}🛑 Stopping test server (PID: $SERVER_PID)${NC}"
        kill $SERVER_PID 2>/dev/null
        wait $SERVER_PID 2>/dev/null
    fi

    # Kill any remaining cargo processes
    pkill -f "cargo run --example test_server" 2>/dev/null

    echo -e "${GREEN}✅ Cleanup complete${NC}"
    exit 0
}

# Set trap to cleanup on script exit
trap cleanup EXIT INT TERM

# Start the test server in background
echo -e "${BLUE}🔧 Starting test server...${NC}"
cargo run --example test_server &
SERVER_PID=$!

# Wait for server to be ready
echo -e "${YELLOW}⏳ Waiting for server to be ready...${NC}"
for i in {1..30}; do
    if curl -s http://localhost:3000 > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Server ready on http://localhost:3000${NC}"
        break
    fi

    if [ $i -eq 30 ]; then
        echo -e "${RED}❌ Server failed to start within 30 seconds${NC}"
        exit 1
    fi

    echo -n "."
    sleep 1
done

echo ""

# Run the tests
echo -e "${BLUE}🧪 Running metadata tests...${NC}"
echo -e "${YELLOW}Note: Server will automatically exit after 60 seconds or when tests complete${NC}"

# Run the real metadata validation test
echo -e "\n${BLUE}📝 Running real metadata validation tests...${NC}"
pnpm run test:metadata:real

# Run cross-browser tests
echo -e "\n${BLUE}🌐 Running cross-browser tests...${NC}"
pnpm run test:metadata:cross-browser

# Generate comprehensive report
echo -e "\n${BLUE}📊 Generating comprehensive report...${NC}"
pnpm run test:metadata:comprehensive

echo -e "\n${GREEN}✅ All tests completed!${NC}"
echo -e "${BLUE}📁 Check the reports directory for detailed results${NC}"
