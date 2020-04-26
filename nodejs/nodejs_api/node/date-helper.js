function timestamp_unix() {
  let ts = Date.now();
  return Math.floor(ts/1000);
}

function timestamp() {
  let ts = Date.now();
  return JSON.stringify(ts);
}


function utc_string() {
  var date = new Date();
  return date.toUTCString();
}

function format_date(locale, weekday, year, month, day, time_zone, time_zone_name) {
  var date = new Date();
  var options = {};
  options.weekday = weekday;
  options.year = year;
  options.month = month;
  options.day = day;
  options.timeZone = time_zone;
  options.timeZoneName = time_zone_name;
  return date.toLocaleDateString(locale, options);
}

module.exports = { timestamp_unix, timestamp, utc_string, format_date }
