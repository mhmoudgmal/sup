exports.handler = (_event, _context, callback) => {
  console.log('Lambda invoked...');
  callback(null, { vars: process.env });
}
