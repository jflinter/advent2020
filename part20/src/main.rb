require 'pry'

SEA_MONSTER = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ".lines.map {|l| l.delete("\n").chars}

class Tile
  attr_reader :key, :contents, :rotation, :flipped
  def initialize(key, contents, rotation, flipped)
    @key = key
    @contents = contents
    @rotation = rotation
    @flipped = flipped
  end

  def self.parse(chunk)
    key = chunk.lines[0].gsub(/\D/, "").to_i
    Tile.new(
      key,
      chunk.lines[1..-1].map(&:strip).map(&:chars),
      0,
      false,
    )
  end

  def self.zip_innards_right(tiles)
    tiles[0].innards.zip(tiles[1..-1].map(&:innards)).map(&:flatten)
  end

  def innards
    contents[1...-1].map {|c| c[1...-1]}
  end

  def rotate(times = 1)
    tile = self
    times.times do
      tile = Tile.new(
        tile.key,
        tile.contents.transpose.map(&:reverse),
        (tile.rotation + 90) % 360,
        tile.flipped,
      )
    end
    tile
  end

  def flip
    Tile.new(
      self.key,
      self.contents.reverse,
      self.rotation,
      !self.flipped
    )
  end

  def to_s
    contents.map(&:join).join("\n")
  end

  def inspect; to_s; end

  def edges
    {
      north: north,
      south: south,
      west: west,
      east: east,
    }
  end

  def match?(edge)
    edges.any? {|e| e == edge}
  end

  def north
    contents[0].join
  end

  def south
    contents[contents.length - 1].join
  end

  def west
    contents.map {|row| row[0]}.join.reverse
  end

  def east
    contents.map {|row| row[row.length - 1]}.join
  end

  def permutations
    (0..3).map {|i| self.rotate(i)}.flat_map {|t| [t, t.flip]}
  end

  def match_count(side, tiles)
    tiles.count do |t|
      t.key != self.key &&
        t.edges.any? {|e| e == self.edges[side]}
    end
  end
end

def main
  input = File.read('../input.txt')
  tiles = input.split("\n\n").map {|chunk| Tile.parse(chunk)}
  size = Math.sqrt(tiles.length).to_i
  placed = []
  all_permutations = tiles.flat_map(&:permutations)
  (0...tiles.count).each do |idx|
    row, col = [idx / size, idx % size]
    tile = if idx == 0
      corners = all_permutations.select do |p1|
        !all_permutations.any? {|p2| p1.key != p2.key && (p1.north == p2.south || p1.west == p2.east)} &&
          all_permutations.any? {|p2| p1.key != p2.key && (p1.south == p2.north || p1.east == p2.west)}
      end
      corners.first
    elsif col == 0
      up_neighbor = placed[idx - size]
      all_permutations.select do |p1|
        p1.key != up_neighbor.key && p1.north == up_neighbor.south
      end.first
    else
      left_neighbor = placed[idx - 1]
      all_permutations.select do |p1|
        p1.key != left_neighbor.key && p1.west.reverse == left_neighbor.east
      end.first
    end
    raise "no tile" unless tile
    placed[idx] = tile
  end

  image = placed.each_slice(size).map do |slice|
    all_innards = slice.map(&:innards)
    head, *tail = all_innards
    head.zip(*tail).map(&:flatten)
  end.reduce(&:+)

  p, m = permutations(image).map do |i|
    [i, find_monsters(i)]
  end.max_by {|_, m| m.count}

  mark_monsters(p, m)

  puts p.map(&:join).reduce(&:+).count('#')
  p.each {|l| puts l.join}
end

def permutations(image)
  x = image
  y = image.reverse
  (0..3).flat_map do |i|
    x = x.transpose.map(&:reverse)
    y = y.transpose.map(&:reverse)
    [x, y]
  end
end

def find_monsters(image)
  sea_monster_points = (0...SEA_MONSTER[0].length).to_a.product((0...SEA_MONSTER.length).to_a).select {|x, y| SEA_MONSTER[y][x] == '#'}
  origins = (0...image[0].length - SEA_MONSTER[0].length).to_a.product((0...image.length - SEA_MONSTER.length).to_a)
  origins.select do |row, col|
    sea_monster_points.all? do |x, y|
      image[col+y][row+x] == '#'
    end
  end
end

def mark_monsters(image, points)
  sea_monster_points = (0...SEA_MONSTER[0].length).to_a.product((0...SEA_MONSTER.length).to_a).select {|x, y| SEA_MONSTER[y][x] == '#'}
  points.each do |x, y|
    sea_monster_points.each do |sx, sy|
      image[sy+y][sx+x] = 'O'
    end
  end
  image
end

main()


# "Tile 2311:\n..##.#..#.\n##..#.....\n#...##..#.\n####.#...#\n##.##.###.\n##...#.###\n.#.#

