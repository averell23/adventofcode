import java.security.*;
import java.math.*;

public class Day4AdventCoins {


  static int md5length(String original) throws Exception {
    byte[] bytesOfMessage = original.getBytes("UTF-8");
    MessageDigest md = MessageDigest.getInstance("MD5");
    byte[] thedigest = md.digest(bytesOfMessage);
    BigInteger bigInt = new BigInteger(1,thedigest);
    return bigInt.toString(16).length();
  }

  public static void main(String[] args) throws Exception {
    String secret = args[0];

    int count = 0;
    int size = 32;

    do {
      count++;
      size = md5length(secret + Integer.toString(count));
    } while (size > 26);

    System.out.println(count);
  }

}
