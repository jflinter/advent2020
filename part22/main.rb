require 'set'

input = <<~HEREDOC
  Player 1:
  9
  2
  6
  3
  1

  Player 2:
  5
  8
  4
  7
  10
HEREDOC
input = File.read("input.txt")
# input = <<~HEREDOC
#   Player 1:
#   43
#   19

#   Player 2:
#   2
#   29
#   14
# HEREDOC

left, right = input.split("\n\n").map {|l| l.lines[1..].map(&:to_i)}

def debug(&blk)
  # puts blk.call
end

@memoized = {}
def memoized_game(left, right, id)
  game_key = "#{left.join(',')} / #{right.join(',')}"
  @memoized[game_key] ||= game(left, right, id)
end

def game(left, right, id)
  round = 1
  debug {"=== Game #{id} ==="}
  puts "=== Game #{id} ==="
  debug {""}
  seen = {}
  while left.any? && right.any?
    debug {"-- Round #{round} (Game #{id}) --"}
    # seen_key = left + [-1] + right
    seen_key = "#{left.join(',')} / #{right.join(',')}"
    if seen[seen_key]
      debug {"bailout, player 1 wins"}
      return [left, right, true]
    end
    seen[seen_key] = true

    ## begin round
    debug {"Player 1's deck: #{left.inspect}"}
    debug {"Player 2's deck: #{right.inspect}"}
    a, b = [left.shift, right.shift]
    debug {"Player 1 plays: #{a}"}
    debug {"Player 1 plays: #{b}"}
    round_winner = if (left.count >= a && right.count >= b)
      debug {"Playing a sub-game to determine the winner..."}
      l, r = memoized_game(left[0...a], right[0...b], id+1)
      l.any? ? left : right
    else
      a > b ? left : right
    end
    ## end round
    if round_winner == left
      debug {"Player 1 wins round #{round} of game #{id}!"}
      debug {""}
      left += [a,b]
    else
      debug {"Player 2 wins round #{round} of game #{id}!"}
      debug {""}
      right += [b,a]
    end
    round += 1
  end
  [left, right, left.count > right.count]
end

left, right, left_won = memoized_game(left, right, 1)

winner = left_won ? left : right

puts winner.zip(winner.count.downto(1)).map {|a,b| a*b}.sum
