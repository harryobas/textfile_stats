# frozen_string_literal: true

RSpec.describe TextfileStats do
  it "has a version number" do
    expect(TextfileStats::VERSION).not_to be nil
  end

  describe '.word_count' do
    it 'returns the number of words in a text file' do 
      file = File.expand_path('spec/fixtures/test_file.txt')
      expect(TextfileStats.word_count(file)).to eq 4
    end
  end
end
