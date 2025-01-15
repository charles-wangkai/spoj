import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);

    while (true) {
      int value = sc.nextInt();
      if (value == 42) {
        break;
      }

      System.out.println(solve(value));
    }

    sc.close();
  }

  static int solve(int value) {
    return value;
  }
}