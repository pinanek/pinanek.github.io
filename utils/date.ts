function getDateString(date: string): string {
  return new Date(date).toLocaleDateString('en-US', {
    day: 'numeric',
    year: 'numeric',
    month: 'short'
  })
}

export { getDateString }
