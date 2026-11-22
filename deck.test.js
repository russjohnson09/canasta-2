const test = require('node:test');
const assert = require('node:assert');
const Deck = require('./deck');

test('Deck - should initialize with 52 cards', () => {
  const deck = new Deck();
  assert.strictEqual(deck.cards.length, 52);
});

test('Deck - should shuffle cards', () => {
  const deck1 = new Deck();
  const deck2 = new Deck();
  deck2.shuffle();
  assert.notDeepStrictEqual(deck1.cards, deck2.cards);
});

test('Deck - should draw specified number of cards', () => {
  const deck = new Deck();
  const drawn = deck.draw(5);
  assert.strictEqual(drawn.length, 5);
  assert.strictEqual(deck.cards.length, 47);
});

test('Deck - should throw error when drawing more cards than available', () => {
  const deck = new Deck();
  assert.throws(() => deck.draw(53), /Not enough cards in the deck/);
});

test('Deck - should contain queen of hearts exactly once when drawn all cards', () => {
  const deck = new Deck();
  deck.shuffle();
  const allCards = deck.draw(52);
  const queenOfHeartsCount = allCards.filter(
    card => card.value === 'Q' && card.suit === 'hearts'
  ).length;
  assert.strictEqual(queenOfHeartsCount, 1);
});
