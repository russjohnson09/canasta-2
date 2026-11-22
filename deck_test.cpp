#define CATCH_CONFIG_MAIN
#include <catch2/catch_test_macros.hpp>
#include "deck.cpp"

TEST_CASE("Deck initializes with 52 cards", "[deck]") {
    Deck deck;
    REQUIRE(deck.cards.size() == 52);
}

TEST_CASE("Deck shuffles correctly", "[deck]") {
    Deck deck1;
    Deck deck2;
    deck2.shuffle();
    REQUIRE(deck1.cards != deck2.cards);
}

TEST_CASE("Deck draws cards correctly", "[deck]") {
    Deck deck;
    auto drawn = deck.draw(5);
    REQUIRE(drawn.size() == 5);
    REQUIRE(deck.cards.size() == 47);
}

TEST_CASE("Deck throws when drawing too many cards", "[deck]") {
    Deck deck;
    REQUIRE_THROWS_AS(deck.draw(53), std::runtime_error);
}

TEST_CASE("Queen of hearts appears exactly once", "[deck]") {
    Deck deck;
    deck.shuffle();
    auto all_cards = deck.draw(52);
    int qh_count = 0;
    for (const auto& card : all_cards) {
        if (card.suit == Suit::Hearts && card.value == Value::Queen) {
            qh_count++;
        }
    }
    REQUIRE(qh_count == 1);
}
