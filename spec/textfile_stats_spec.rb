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
  context 'when file is empty' do 
    it 'raises error' do 
      file = File.expand_path('spec/fixtures/empty_test_file.txt')
      expect{TextfileStats.word_count(file)}.to raise_error(StandardError)
    end
  end
  context 'when file is not found' do
    it 'raises error' do
      file = File.expand_path('spec/fixtires/no_text_file.txt')
      expect{TextfileStats.word_count(file)}.to raise_error(IOError)
    end
  end
end
