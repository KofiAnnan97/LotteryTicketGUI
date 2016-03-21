package LotteryStuff;

import javax.swing.*;
import java.awt.*;
import java.awt.event.*;

public class LottoTicket {
	static JFrame frame;
	static JPanel panel;
	private JButton genb, quitb;
	private JLabel questionl;
	private JMenuBar menuBar;
	int width = 400, height = 400;

	public LottoTicket() {
		init();
	}

	private void init() {
		frame = new JFrame("Lottery Number Generator");
		frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
		frame.setLocationRelativeTo(null);
		frame.setVisible(true);
		frame.pack();
		frame.setSize(width, height);
		frame.setMinimumSize(new Dimension(400, 400));
		// frame.setMaximumSize(new Dimension(500, 500));
		
		menu();
		mainWindow();

	}

	private void menu() {
		menuBar = new JMenuBar();
		frame.setJMenuBar(menuBar);

		JMenu home = new JMenu("Home");
		menuBar.add(home);
		
		JMenu presets = new JMenu("Presets");
		menuBar.add(presets);

		JMenuItem addPresets = new JMenuItem("Add New Preset");
		presets.add(addPresets);
		

		JMenu recentPresets = new JMenu("Recent Presets...");
		presets.add(recentPresets);

		JMenuItem editPresets = new JMenuItem("Edit Presets");
		presets.add(editPresets);

		JMenu setting = new JMenu("Settings");
		menuBar.add(setting);

		JMenuItem about = new JMenuItem("About");
		setting.add(about);

		JMenuItem preferences = new JMenuItem("Preferences");
		setting.add(preferences);

		JMenuItem license = new JMenuItem("License of Use");
		setting.add(license);
	}

	private void mainWindow() {
		panel = new JPanel(new GridBagLayout());
		panel.setBackground(Color.WHITE);
		frame.add(panel);

		GridBagConstraints gbc = new GridBagConstraints();

		genb = new JButton("Generate");
		genb.addActionListener(new ActionListener() {
			public void actionPerformed(ActionEvent e) {
				frame.remove(panel);
				new NumberDisplay();
			}
		});
		gbc.gridx = 0;
		gbc.gridy = 1;
		panel.add(genb, gbc);

		quitb = new JButton("Quit");
		quitb.addActionListener(new ActionListener() {
			public void actionPerformed(ActionEvent e) {
				System.exit(0);
			}
		});
		gbc.gridx = 0;
		gbc.gridy = 2;
		panel.add(quitb, gbc);

		questionl = new JLabel("Would you like to generate a lottery ticket?");
		gbc.gridx = 0;
		gbc.gridy = 0;
		panel.add(questionl, gbc);

		frame.revalidate();
		frame.repaint();

	}

	public static void main(String[] args) {
		new LottoTicket();
	}

}
