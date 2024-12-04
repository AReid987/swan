import React, { useRef, useState, useEffect } from 'react';
import { Button } from './ui/button';
import Send from './ui/Send';
import Stop from './ui/Stop';

interface InputProps {
  handleSubmit: (e: React.FormEvent) => void;
  disabled?: boolean;
  isLoading?: boolean;
  onStop?: () => void;
}

export default function Input({
  handleSubmit,
  disabled = false,
  isLoading = false,
  onStop
}: InputProps) {
  const [value, setValue] = useState('');
  const textAreaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (textAreaRef.current && !disabled) {
      textAreaRef.current.focus();
    }
  }, [disabled, value]);

  const useAutosizeTextArea = (textAreaRef: HTMLTextAreaElement | null, value: string) => {
    useEffect(() => {
      if (textAreaRef) {
        textAreaRef.style.height = "0px"; // Reset height
        const scrollHeight = textAreaRef.scrollHeight;
        textAreaRef.style.height = Math.min(scrollHeight, maxHeight) + "px";
      }
    }, [textAreaRef, value]);
  };

  const minHeight = "1rem";
  const maxHeight = 10 * 24;

  useAutosizeTextArea(textAreaRef.current, value);

  const handleChange = (evt: React.ChangeEvent<HTMLTextAreaElement>) => {
    const val = evt.target?.value;
    setValue(val);
  };

  const handleKeyDown = (evt: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (evt.key === 'Enter' && !evt.shiftKey) {
      evt.preventDefault();
      if (value.trim()) {
        handleSubmit(new CustomEvent('submit', { detail: { value } }));
        setValue('');
      }
    }
  };

  const onFormSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (value.trim()) {
      handleSubmit(new CustomEvent('submit', { detail: { value } }));
      setValue('');
    }
  };

  return (
    <form onSubmit={onFormSubmit} className="flex relative bg-white h-auto px-[16px] pr-[38px] py-[1rem]">
      <textarea
        autoFocus
        id="dynamic-textarea"
        placeholder="What should goose do?"
        value={value}
        onChange={handleChange}
        onKeyDown={handleKeyDown}
        disabled={disabled}
        ref={textAreaRef}
        rows={1}
        style={{
          minHeight: `${minHeight}px`,
          maxHeight: `${maxHeight}px`,
          overflowY: 'auto'
        }}
        className={`w-full outline-none border-none focus:ring-0 bg-transparent p-0 text-14 resize-none ${
          disabled ? 'cursor-not-allowed opacity-50' : ''
        }`}
      />
      {isLoading ? (
        <Button
          type="button"
          size="icon"
          variant="ghost"
          onClick={onStop}
          className="absolute right-2 top-1/2 -translate-y-1/2 text-indigo-600 hover:text-indigo-700 hover:bg-indigo-100"
        >
          <Stop size={24} />
        </Button>
      ) : (
        <Button
          type="submit"
          size="icon"
          variant="ghost"
          disabled={disabled || !value.trim()}
          className={`absolute right-2 top-1/2 -translate-y-1/2 text-indigo-600 hover:text-indigo-700 hover:bg-indigo-100 ${
            disabled || !value.trim() ? 'opacity-50 cursor-not-allowed' : ''
          }`}
        >
          <Send size={24} />
        </Button>
      )}
    </form>
  );
}