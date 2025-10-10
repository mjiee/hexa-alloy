// format number
export const formatNumber = (num?: number, decimals = 2): string => {
  if (num === undefined || num === null) return "--";
  return num.toFixed(decimals);
};

// format large number
export const formatLargeNumber = (num?: number): string => {
  if (num === undefined || num === null) return "--";
  if (num >= 100000000) return `${(num / 100000000).toFixed(2)}亿`;
  if (num >= 10000) return `${(num / 10000).toFixed(2)}万`;
  return num.toFixed(0);
};

// format date time
export const formatDateTime = (dateStr?: string, timeStr?: string): string => {
  if (!dateStr) return "--";
  const date = dateStr.replace(/-/g, "/");
  return timeStr ? `${date} ${timeStr}` : date;
};
