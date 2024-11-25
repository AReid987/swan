import * as linkify from 'linkifyjs';

// Helper to normalize URLs for comparison
function normalizeUrl(url: string): string {
  try {
    const parsed = new URL(url.toLowerCase());
    // Remove trailing slashes and normalize protocol
    return `${parsed.protocol}//${parsed.host}${parsed.pathname.replace(/\/$/, '')}${parsed.search}${parsed.hash}`;
  } catch {
    // If URL parsing fails, just lowercase it
    return url.toLowerCase();
  }
}

export function extractUrls(content: string, previousUrls: string[] = []): string[] {
  // Use linkifyjs to find URLs
  const links = linkify.find(content);
  
  // Get URLs from current content
  const currentUrls = links
    .filter(link => link.type === 'url')
    .map(link => link.href);
  
  // Normalize all URLs for comparison
  const normalizedPreviousUrls = previousUrls.map(normalizeUrl);
  const normalizedCurrentUrls = currentUrls.map(url => {
    const normalized = normalizeUrl(url);
    console.log('Normalizing URL:', { original: url, normalized });
    return normalized;
  });
  
  // Filter out duplicates
  const uniqueUrls = currentUrls.filter((url, index) => {
    const normalized = normalizedCurrentUrls[index];
    const isDuplicate = normalizedPreviousUrls.some(prevUrl => 
      normalizeUrl(prevUrl) === normalized
    );
    console.log('URL comparison:', { 
      url, 
      normalized,
      previousUrls: normalizedPreviousUrls,
      isDuplicate 
    });
    return !isDuplicate;
  });
  
  console.log('Content:', content);
  console.log('Found URLs:', uniqueUrls);
  console.log('Previous URLs:', previousUrls);
  
  return uniqueUrls;
}