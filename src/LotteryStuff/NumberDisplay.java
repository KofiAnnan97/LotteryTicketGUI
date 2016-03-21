package LotteryStuff;

import java.awt.*;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import javax.swing.*;

public class NumberDisplay {
	private JPanel panel2;
	private JButton playAgainb, returnb;
	private JTextField num1, num2, num3, num4, num5, num6;
	int[] mega = new int[6];

	public NumberDisplay() {
		init();
	}

	private void init() {
		panel2 = new JPanel(new GridBagLayout());
		panel2.setBackground(Color.WHITE);
		LottoTicket.frame.add(panel2);

		numGenerator(mega);

		GridBagConstraints gbc = new GridBagConstraints();

		num1 = new JTextField(Integer.toString(mega[0]));
		num1.setEditable(false);
		gbc.gridx = 0;
		gbc.gridy = 1;
		panel2.add(num1, gbc);

		num2 = new JTextField(Integer.toString(mega[1]));
		num2.setEditable(false);
		gbc.gridx = 1;
		gbc.gridy = 1;
		panel2.add(num2, gbc);

		num3 = new JTextField(Integer.toString(mega[2]));
		num3.setEditable(false);
		gbc.gridx = 2;
		gbc.gridy = 1;
		panel2.add(num3, gbc);

		num4 = new JTextField(Integer.toString(mega[3]));
		num4.setEditable(false);
		gbc.gridx = 3;
		gbc.gridy = 1;
		panel2.add(num4, gbc);

		num5 = new JTextField(Integer.toString(mega[4]));
		num5.setEditable(false);
		gbc.gridx = 4;
		gbc.gridy = 1;
		panel2.add(num5, gbc);

		num6 = new JTextField(Integer.toString(mega[5]));
		num6.setEditable(false);
		gbc.gridx = 5;
		gbc.gridy = 1;
		panel2.add(num6, gbc);

		playAgainb = new JButton("Play Again");
		playAgainb.addActionListener(new ActionListener() {
			public void actionPerformed(ActionEvent e) {

				numGenerator(mega);
				num1.setText(Integer.toString(mega[0]));
				num2.setText(Integer.toString(mega[1]));
				num3.setText(Integer.toString(mega[2]));
				num4.setText(Integer.toString(mega[3]));
				num5.setText(Integer.toString(mega[4]));
				num6.setText(Integer.toString(mega[5]));
				panel2.revalidate();
			}
		});
		gbc.gridx = 6;
		gbc.gridy = 1;
		panel2.add(playAgainb, gbc);

		returnb = new JButton("Return");
		returnb.addActionListener(new ActionListener() {
			public void actionPerformed(ActionEvent e) {
				LottoTicket.frame.remove(panel2);
				LottoTicket.frame.add(LottoTicket.panel);
				LottoTicket.frame.repaint();
			}
		});
		gbc.gridx = 6;
		gbc.gridy = 2;
		panel2.add(returnb, gbc);

		LottoTicket.frame.revalidate();
	}

	public static int[] numGenerator(int[] nums) {
		for (int i = 0; i < nums.length; i++) {
			if(i == 5)   nums[i] = (int) (Math.random() * 15 + 1);
	
			else   nums[i] = (int) (Math.random() * 75 + 1);
		}
		return nums;
	}
}
