# In this game, the players take turns saying numbers. They begin by taking turns reading from a list of starting numbers (your puzzle input). Then, each turn consists of considering the most recently spoken number:
#     If that was the first time the number has been spoken, the current player says 0.
#     Otherwise, the number had been spoken before; the current player announces how many turns apart the number is from when it was previously spoken.

class Game
  attr_reader :starting, :turn, :cache, :last_num, :turns
  def initialize(starting)
    @starting = starting
    @turn = 0
    @cache = {} # value to array of turns it was seen on
    @last_num = nil
    @turns = []
  end

  def incr
    @last_num = value
    @cache[@last_num] ||= []
    @cache[@last_num] = [@cache.dig(@last_num, 1), turn]
    @turn += 1
  end

  def value
    if turn < starting.length
      return starting[turn]
    elsif cache[last_num][0] == nil
      0
    else
      cache[last_num][1] - cache[last_num][0]
    end
  end

  def self.debug(&blk)
    # puts x
  end

  def self.val_on_nth_turn(n, starting)
    game = Game.new(starting)
    (0...n).each do |i|
      if i % 100 == 0
        puts "turn #{i}"
      end
      debug { "TURN #{i + 1}" }
      debug { "cache is #{game.cache}" }
      debug { "last num is #{game.last_num}" }
      game.incr
      debug { "saying #{game.last_num}" }
      debug { "turns: #{game.turns}" }
      debug { "---" }
    end
    game.last_num
  end

  def self.assert_eq(left, right)
    raise "expected #{left}, got #{right}" unless left == right
  end

  def self.test
    # assert_eq(3, Game.val_on_nth_turn(6, [0,3,6]))
    expected = [0, 3, 6, 0, 3, 3, 1, 0, 4, 0]
    expected.each_with_index do |e, i|
      assert_eq(e, Game.val_on_nth_turn(i + 1, [0,3,6]))
      debug { "ok: #{i}" }
    end
    assert_eq(436, Game.val_on_nth_turn(2020, [0,3,6]))
    puts "OK"
  end

  def self.main
    puts Game.val_on_nth_turn(30_000_000, [0,5,4,1,10,14,7])
  end
end

Game.test
Game.main