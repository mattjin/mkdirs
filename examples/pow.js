var mkdirs = require('../lib');

mkdirs('./tmp/foo/bar', function (err) {
  if (err) {
    console.error(err);
  }
});
