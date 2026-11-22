#include <vector>
#include <string>
#include <algorithm>
#include <random>
#include <stdexcept>

enum class Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs
};

enum class Value {
    Ace, Two, Three, Four, Five, Six, Seven,
    Eight, Nine, Ten, Jack, Queen, King
};

struct Card {
    Suit suit;
    Value value;

    bool operator==(const Card& other) const {
        return suit == other.suit && value == other.value;
    }
};

class Deck {
public:
    std::vector<Card> cards;

    Deck() {
        Suit suits[] = {Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs};
        Value values[] = {
            Value::Ace, Value::Two, Value::Three, Value::Four,
            Value::Five, Value::Six, Value::Seven, Value::Eight,
            Value::Nine, Value::Ten, Value::Jack, Value::Queen, Value::King
        };

        for (auto s : suits) {
            for (auto v : values) {
                cards.push_back({s, v});
            }
        }
    }

    void shuffle() {
        std::random_device rd;
        std::mt19937 g(rd());
        std::shuffle(cards.begin(), cards.end(), g);
    }

    std::vector<Card> draw(size_t count) {
        if (count > cards.size()) {
            throw std::runtime_error("Not enough cards in the deck");
        }
        std::vector<Card> drawn(cards.begin(), cards.begin() + count);
        cards.erase(cards.begin(), cards.begin() + count);
        return drawn;
    }
};
