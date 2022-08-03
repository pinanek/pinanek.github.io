import { isValid, parse } from 'date-fns'

function isPostValidDate(date: string) {
  return isValid(parse(date, 'MM-dd-yyyy', new Date()))
}

function getPostDateString(date: string): string {
  return new Date(date).toLocaleDateString('en-US', {
    day: 'numeric',
    year: 'numeric',
    month: 'short'
  })
}

export { isPostValidDate, getPostDateString }
