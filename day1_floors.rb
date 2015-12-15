def new_floor(old_floor, c)
  if c == '('
    old_floor + 1
  elsif c == ')'
    old_floor - 1
  else
    old_floor
  end
end

def compute_floor(string)
  string.chars.inject(0) { |floor,c| new_floor(floor, c) }
end

def find_basement(string)
  floor, count = string.chars.inject([0,0]) do |m, c|
    floor, count = m
    return count if floor == -1
    [new_floor(floor, c), count + 1]
  end

  floor == -1 ? count : -1
end

puts find_basement(ARGV.first).to_s
