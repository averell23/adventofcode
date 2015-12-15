class Box

  attr_reader :l, :w, :h

  def initialize(dimensions)
    @l, @w, @h = dimensions.split('x').map(&:to_i)
  end

  def areas
    [ 2*l*w, 2*w*h, 2*h*l ]
  end

  def paper_size
    areas.inject(:+) + (areas.min/2)
  end

  def ribbon
    smallest = [ l, w, h ].sort[0..1]
    smallest.inject(:+) * 2
  end

  def volume
    l * w * h
  end
end

File.open(ARGV.first) do |f|
  size = f.lines.inject(0) { |sum, line| sum + Box.new(line).ribbon + Box.new(line).volume }
  puts size.to_s
end
