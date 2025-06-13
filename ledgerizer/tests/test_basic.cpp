#include "ledgerizer/ledgerizer.h"

#include <cassert>
#include <iostream>

int main() {
    std::vector<uint8_t> original = {1,1,1,2,2,3,3,3,3,4};

    auto ledger = ledgerizer::encode(original);
    auto recovered = ledgerizer::decode(ledger);

    assert(original == recovered && "Round-trip failed");

    auto buffer = ledgerizer::serialize(ledger);
    auto ledger2 = ledgerizer::deserialize(buffer);
    auto recovered2 = ledgerizer::decode(ledger2);

    assert(original == recovered2 && "Serialize/deserialize failed");

    std::cout << "All ledgerizer basic tests passed.\n";
    return 0;
} 