const convertDateToString = (date: Date | string | undefined): string => {
    if (!date) return "No Date Found";

    // If the date is a string, try to parse it into a Date object
    const parsedDate = typeof date === "string" ? new Date(date) : date;

    // Check if the parsedDate is a valid Date object
    if (isNaN(parsedDate.getTime())) return "Invalid Date";

    return parsedDate.toISOString().split('T')[0]; // Format date to YYYY-MM-DD
};

export default convertDateToString