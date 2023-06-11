# frozen_string_literal: true

RSpec.describe TextfileStats do

  let(:file) { File.expand_path('spec/fixtures/test_file.txt') }
  let(:empty_file) { File.expand_path('spec/fixtures/empty_test_file.txt')}

  it "has a version number" do
    expect(TextfileStats::VERSION).not_to be nil
  end

  describe '.words_count' do
    it 'returns the number of words in a text file' do 
      expect(TextfileStats.words_count(file)).to eq 322392
    end
  end
  context 'when file is empty' do 
    it 'raises error' do 
      expect{TextfileStats.words_count(empty_file)}.to raise_error(StandardError)
    end
  end
  context 'when file is not found' do
    it 'raises error' do
      file = File.expand_path('spec/fixtires/no_text_file.txt')
      expect{TextfileStats.words_count(file)}.to raise_error(IOError)
    end
  end
  describe 'chars_count' do 
    it 'returns the number of characters in a text file' do 
      expect(TextfileStats.chars_count(file)).to eq  1842499

    end
  end
end
