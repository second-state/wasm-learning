const { resize_file, create_sqlite, query_sqlite, fetch, download, show_now, utc_now, my_time } = require('../pkg/nodejs_example.js');
  
(async () => {
  const dim = {
    width: 100,
    height: 100
  };

  resize_file(JSON.stringify([dim, 'cat.png', `./cat-${dim.width}-${dim.height}.png`]));

  create_sqlite();
  query_sqlite();

  fetch("https://raw.githubusercontent.com/angular/angular.js/master/LICENSE");
  download("https://www.secondstate.io/", "secondstate.html");

  show_now();
  utc_now();
  my_time("America/Chicago");

})();
