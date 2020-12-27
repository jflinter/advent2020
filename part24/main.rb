require 'pry'

class World
  attr_reader :tiles, :min_x, :min_y, :max_x, :max_y

  def initialize
    @tiles = {}
    @min_x = 9999999
    @min_y = 9999999
    @max_x = -9999999
    @max_y = -9999999
  end

  def flip_tile(pos)
    @tiles[pos] = !@tiles[pos]
    if @tiles[pos]
      x, y = pos
      @max_x = [max_x, x].max
      @max_y = [max_y, y].max
      @min_x = [min_x, x].min
      @min_y = [min_y, y].min
    end
  end

  def self.position_from(pos, dir)
    x, y = pos
    case dir
    when 'e'
      [x+1, y]
    when 'w'
      [x-1, y]
    when 'ne'
      y % 2 == 0 ? [x, y-1] : [x+1, y-1]
    when 'nw'
      y % 2 == 0 ? [x-1, y-1] : [x, y-1]
    when 'se'
      y % 2 == 0 ? [x, y+1] : [x+1, y+1]
    when 'sw'
      y % 2 == 0 ? [x-1, y+1] : [x, y+1]
    else raise 'bad'
    end
  end

  def self.parse(input)
    world = World.new
    all_tiles = input.split.map do |line|
      i = 0
      buf = ''
      dirs = []
      while i < line.length
        case line[i]
        when 'n'
          buf = 'n'
        when 'e'
          dirs << (buf + 'e')
          buf = ''
        when 's'
          buf = 's'
        when 'w'
          dirs << (buf + 'w')
          buf = ''
        else raise 'unexpected char'
        end
        i += 1
      end
      dirs
    end
    all_tiles.each do |tile|
      pos = [0, 0]
      tile.each do |dir|
        pos = position_from(pos, dir)
      end
      world.flip_tile(pos)
    end
    world
  end

  def count
    @tiles.count {|_,v| v}
  end

  def self.neighbors(pos)
    ['e', 'ne', 'nw', 'w', 'sw', 'se'].map {|d| position_from(pos, d)}
  end

  def incr(i=1)
    old_world = self
    i.times do
      new_world = World.new
      ((old_world.min_x-2)..(old_world.max_x+2)).to_a.product(((old_world.min_y-2)..(old_world.max_y+2)).to_a).each do |pos|
        neighbor_count = World.neighbors(pos).count {|p| old_world.tiles[p]}
        wants_black = if old_world.tiles[pos]
          neighbor_count == 1 || neighbor_count == 2
        else
          neighbor_count == 2
        end
        new_world.flip_tile(pos) if wants_black
      end
      old_world = new_world
    end
    old_world
  end
end

def main
  input = 'sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew'
  input = File.read('input.txt')
  world = World.parse(input)
  puts world.count
  puts world.incr.count
  puts world.incr(100).count
end

main()