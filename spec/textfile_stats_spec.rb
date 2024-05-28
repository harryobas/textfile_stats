# frozen_string_literal: true

require 'spec_helper'

RSpec.describe TextfileStats do

  let(:file) { File.expand_path('spec/fixtures/test_file.txt') }
  let(:empty_file) { File.expand_path('spec/fixtures/empty_test_file.txt')}
  let(:not_text_file) { File.expand_path('spec/fixtures/not_text_test_file.pdf')}

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
      file = File.expand_path('spec/fixtures/no_text_file.txt')
      expect{TextfileStats.words_count(file)}.to raise_error(IOError)
    end
  end
  context 'when file is not a text file' do
    it 'raises error' do
      expect{TextfileStats.words_count(not_text_file)}.to raise_error(StandardError)
    end
  end
  describe '.chars_count' do 
    it 'returns the number of characters in a text file' do 
      expect(TextfileStats.chars_count(file)).to eq  1842499
    end
  end
  context 'when file is empty' do
    it 'raises error' do 
      expect{TextfileStats.chars_count(empty_file)}.to raise_error(StandardError)
    end
  end
  context 'when file is not found' do
    it 'raises error' do
      file = File.expand_path('spec/fixtures/no_text_file.txt')
      expect{TextfileStats.chars_count(file)}.to raise_error(IOError)
    end
  end
  context 'when file is not a text file' do
    it 'raises error' do
      expect{TextfileStats.chars_count(not_text_file)}.to raise_error(StandardError)
    end
  end
  describe '.lines_count' do
    it 'returns the number of lines in a text file' do 
      expect(TextfileStats.lines_count(file)).to eq 5697 
    end 
  end
  
  describe '.word_count' do
    it 'counts the number of a single word in a text file' do 
      expect(TextfileStats.word_count(file, 'Lorem' )).to eq 281
    end
  end

  describe '.char_count' do
    it 'counts the number of a single character in a text file' do 
      expect(TextfileStats.char_count(file, 'e')).to eq 203464
    end
  end
  
end
