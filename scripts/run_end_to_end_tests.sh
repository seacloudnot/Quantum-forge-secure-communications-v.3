#!/bin/bash

# End-to-End Messaging and Data Transmission Test Runner
# Comprehensive test suite for user workflows and data transmission scenarios

set -e

echo "🚀 Starting End-to-End Messaging and Data Transmission Tests"
echo "============================================================"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Test categories
declare -a TEST_CATEGORIES=(
    "end_to_end_messaging_tests"
    "data_transmission_integration_tests"
    "user_workflow_tests"
)

# Individual test functions
declare -a E2E_MESSAGING_TESTS=(
    "test_basic_user_messaging_workflow"
    "test_multi_user_group_messaging"
    "test_large_data_transmission"
    "test_multimedia_data_types"
    "test_concurrent_messaging_load"
    "test_message_ordering_and_delivery"
    "test_error_recovery_and_resilience"
    "test_real_world_usage_patterns"
    "test_performance_under_load"
)

declare -a DATA_TRANSMISSION_TESTS=(
    "test_structured_data_transmission"
    "test_binary_data_patterns"
    "test_protocol_edge_cases"
    "test_data_integrity_verification"
)

declare -a USER_WORKFLOW_TESTS=(
    "test_new_user_onboarding_workflow"
    "test_business_team_collaboration"
    "test_emergency_communication_protocol"
)

# Function to run a specific test category
run_test_category() {
    local category=$1
    local test_file="tests/${category}.rs"
    
    echo -e "${BLUE}📋 Running Test Category: ${category}${NC}"
    echo "----------------------------------------"
    
    if [ ! -f "$test_file" ]; then
        echo -e "${RED}❌ Test file not found: $test_file${NC}"
        return 1
    fi
    
    echo -e "${CYAN}🔍 Compiling tests...${NC}"
    if ! cargo test --test "$category" --no-run; then
        echo -e "${RED}❌ Compilation failed for $category${NC}"
        return 1
    fi
    
    echo -e "${CYAN}🧪 Executing tests...${NC}"
    if cargo test --test "$category" -- --nocapture; then
        echo -e "${GREEN}✅ All tests passed in $category${NC}"
        return 0
    else
        echo -e "${RED}❌ Some tests failed in $category${NC}"
        return 1
    fi
}

# Function to run individual test
run_individual_test() {
    local category=$1
    local test_name=$2
    
    echo -e "${PURPLE}🎯 Running Individual Test: ${test_name}${NC}"
    
    if cargo test --test "$category" "$test_name" -- --nocapture; then
        echo -e "${GREEN}✅ Test passed: $test_name${NC}"
        return 0
    else
        echo -e "${RED}❌ Test failed: $test_name${NC}"
        return 1
    fi
}

# Function to display test summary
display_test_summary() {
    local passed=$1
    local failed=$2
    local total=$((passed + failed))
    
    echo ""
    echo "============================================================"
    echo -e "${CYAN}📊 TEST EXECUTION SUMMARY${NC}"
    echo "============================================================"
    echo -e "Total Tests: ${BLUE}$total${NC}"
    echo -e "Passed: ${GREEN}$passed${NC}"
    echo -e "Failed: ${RED}$failed${NC}"
    
    if [ $failed -eq 0 ]; then
        echo -e "${GREEN}🎉 ALL TESTS PASSED! 🎉${NC}"
        echo "The secure communications system is ready for production!"
    else
        echo -e "${YELLOW}⚠️  Some tests failed. Please review the output above.${NC}"
    fi
    
    echo "============================================================"
}

# Main execution function
main() {
    local start_time=$(date +%s)
    local passed_tests=0
    local failed_tests=0
    local run_mode=${1:-"all"}
    
    echo -e "${CYAN}🔧 Build Environment Check${NC}"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo "Test mode: $run_mode"
    echo ""
    
    # Clean build first
    echo -e "${CYAN}🧹 Cleaning previous builds...${NC}"
    cargo clean
    
    # Build the library
    echo -e "${CYAN}🔨 Building secure communications library...${NC}"
    if ! cargo build --lib; then
        echo -e "${RED}❌ Library build failed${NC}"
        exit 1
    fi
    
    case $run_mode in
        "all")
            echo -e "${YELLOW}🚀 Running ALL end-to-end tests${NC}"
            echo ""
            
            for category in "${TEST_CATEGORIES[@]}"; do
                echo ""
                if run_test_category "$category"; then
                    case $category in
                        "end_to_end_messaging_tests")
                            passed_tests=$((passed_tests + ${#E2E_MESSAGING_TESTS[@]}))
                            ;;
                        "data_transmission_integration_tests")
                            passed_tests=$((passed_tests + ${#DATA_TRANSMISSION_TESTS[@]}))
                            ;;
                        "user_workflow_tests")
                            passed_tests=$((passed_tests + ${#USER_WORKFLOW_TESTS[@]}))
                            ;;
                    esac
                else
                    case $category in
                        "end_to_end_messaging_tests")
                            failed_tests=$((failed_tests + ${#E2E_MESSAGING_TESTS[@]}))
                            ;;
                        "data_transmission_integration_tests")
                            failed_tests=$((failed_tests + ${#DATA_TRANSMISSION_TESTS[@]}))
                            ;;
                        "user_workflow_tests")
                            failed_tests=$((failed_tests + ${#USER_WORKFLOW_TESTS[@]}))
                            ;;
                    esac
                fi
                echo ""
            done
            ;;
            
        "messaging")
            echo -e "${YELLOW}🚀 Running messaging tests only${NC}"
            if run_test_category "end_to_end_messaging_tests"; then
                passed_tests=${#E2E_MESSAGING_TESTS[@]}
            else
                failed_tests=${#E2E_MESSAGING_TESTS[@]}
            fi
            ;;
            
        "data")
            echo -e "${YELLOW}🚀 Running data transmission tests only${NC}"
            if run_test_category "data_transmission_integration_tests"; then
                passed_tests=${#DATA_TRANSMISSION_TESTS[@]}
            else
                failed_tests=${#DATA_TRANSMISSION_TESTS[@]}
            fi
            ;;
            
        "workflow")
            echo -e "${YELLOW}🚀 Running user workflow tests only${NC}"
            if run_test_category "user_workflow_tests"; then
                passed_tests=${#USER_WORKFLOW_TESTS[@]}
            else
                failed_tests=${#USER_WORKFLOW_TESTS[@]}
            fi
            ;;
            
        "quick")
            echo -e "${YELLOW}🚀 Running quick test subset${NC}"
            echo ""
            
            # Run a subset of critical tests
            local quick_tests=(
                "end_to_end_messaging_tests::test_basic_user_messaging_workflow"
                "data_transmission_integration_tests::test_structured_data_transmission"
                "user_workflow_tests::test_new_user_onboarding_workflow"
            )
            
            for test in "${quick_tests[@]}"; do
                IFS='::' read -ra ADDR <<< "$test"
                local category="${ADDR[0]}"
                local test_name="${ADDR[1]}"
                
                if run_individual_test "$category" "$test_name"; then
                    passed_tests=$((passed_tests + 1))
                else
                    failed_tests=$((failed_tests + 1))
                fi
            done
            ;;
            
        *)
            echo -e "${RED}❌ Invalid run mode: $run_mode${NC}"
            echo "Usage: $0 [all|messaging|data|workflow|quick]"
            exit 1
            ;;
    esac
    
    # Performance benchmarks (optional)
    if [ "$run_mode" = "all" ] || [ "$2" = "--with-benchmarks" ]; then
        echo ""
        echo -e "${CYAN}🏃‍♂️ Running Performance Benchmarks${NC}"
        echo "----------------------------------------"
        
        if cargo bench --bench comprehensive_performance_benchmarks; then
            echo -e "${GREEN}✅ Performance benchmarks completed${NC}"
        else
            echo -e "${YELLOW}⚠️  Performance benchmarks had issues (non-critical)${NC}"
        fi
    fi
    
    # Calculate execution time
    local end_time=$(date +%s)
    local execution_time=$((end_time - start_time))
    
    echo ""
    echo -e "${CYAN}⏱️  Total execution time: ${execution_time} seconds${NC}"
    
    # Display final summary
    display_test_summary $passed_tests $failed_tests
    
    # Exit with appropriate code
    if [ $failed_tests -eq 0 ]; then
        exit 0
    else
        exit 1
    fi
}

# Help function
show_help() {
    echo "End-to-End Messaging and Data Transmission Test Runner"
    echo ""
    echo "Usage: $0 [MODE] [OPTIONS]"
    echo ""
    echo "MODES:"
    echo "  all       Run all end-to-end tests (default)"
    echo "  messaging Run messaging tests only"
    echo "  data      Run data transmission tests only"
    echo "  workflow  Run user workflow tests only"
    echo "  quick     Run quick test subset"
    echo ""
    echo "OPTIONS:"
    echo "  --with-benchmarks  Include performance benchmarks"
    echo "  --help            Show this help message"
    echo ""
    echo "EXAMPLES:"
    echo "  $0                    # Run all tests"
    echo "  $0 messaging          # Run messaging tests only"
    echo "  $0 all --with-benchmarks  # Run all tests with benchmarks"
    echo "  $0 quick              # Run quick test subset"
    echo ""
    echo "TEST CATEGORIES:"
    echo "  📧 End-to-End Messaging Tests (${#E2E_MESSAGING_TESTS[@]} tests)"
    echo "  📊 Data Transmission Integration Tests (${#DATA_TRANSMISSION_TESTS[@]} tests)"
    echo "  👥 User Workflow Tests (${#USER_WORKFLOW_TESTS[@]} tests)"
    echo ""
}

# Parse command line arguments
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    show_help
    exit 0
fi

# Run main function
main "$@" 