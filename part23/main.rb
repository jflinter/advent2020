input = [3,8,9,1,2,5,4,6,7]
# input = [3,1,8,9,4,6,5,7,2]

input = input + ((input.max+1)..1_000_000).to_a

def move(cups)
  hand = cups[1...4]
  remaining = [cups[0]] + cups[4..]
  # puts "hand: #{hand.inspect}"
  # puts "remaining: #{remaining.inspect}"
  destination = remaining[0]
  loop do
    destination -= 1
    if destination < 1
      destination = cups.count - 1
    end
    # puts "destination: #{destination}"
    break if !hand.include?(destination)
  end
  idx = remaining.index(destination)
  remaining[1..idx] + hand + remaining[(idx+1)..] + [remaining[0]]
end

c = input
iter = 10_000_000
iter.times do |i|
  puts i if i % 100 == 0
  c = move(c)
end

while c.last != 1 do
  c = c.rotate
end
puts c[...-1].join

