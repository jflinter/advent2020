def debug(str)
  # puts str
end

def get(ary, i)
  return nil if !ary || i < 0 || i >= ary.length
  ary[i]
end

def neighbors(lines, x, y)
  width = lines[0].length
  height = lines.length
  vecs = [-1, 0, 1].product([-1, 0, 1]) - [[0, 0]]
  # vecs = [[1, 0]]
  vecs.count do |vec|
    debug "vec: #{vec}"
    point = [x, y]
    val = '.'
    while val == '.' do
      point = [point, vec].transpose.map(&:sum)
      row = get(lines, point[1])
      val = get(row&.chars, point[0])
      debug "point: #{point}, val: #{val}"
    end
    val == '#'
  end
end

def mutate(lines)
  width = lines[0].length
  height = lines.length
  lines.each_with_index.map do |line, row|
    line.strip.chars.each_with_index.map do |c, col|
      n = neighbors(lines, col, row)
      case c
      when '.' then '.'
      when 'L' then n == 0 ? '#' : 'L'
      when '#' then n >= 5 ? 'L' : '#'
      else raise "bad: #{c}"
      end
    end.join
  end
end

input = <<~INPUT
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
INPUT

x = input.lines

x = File.readlines('../input.txt')

# puts neighbors(x, 0, 0) == 3
# puts neighbors(x, 1, 1) == 0
# puts input
# puts input.lines.count
# puts neighbors(input.lines, 3, 1) == 1

while true do
  y = mutate(x)
  debug ""
  debug "---"
  debug ""
  debug y
  break if y == x
  x = y
end

puts x
puts x.join.count('#')
