extern crate gurnal;

#[cfg(test)]
mod tests {

    use gurnal;

    #[test]
    fn is_recurring() {
      assert!(gurnal::parse("every1day@example.com"));
      assert!(gurnal::parse("1everyday@example.com"));
      assert!(gurnal::parse("1dayevery@example.com"));
      assert!(gurnal::parse("*1day@example.com"));
      assert!(gurnal::parse("1*day@example.com"));
      assert!(gurnal::parse("1day*@example.com"));
  }
}