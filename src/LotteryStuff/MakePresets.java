package LotteryStuff;

import java.awt.GridLayout;

import javax.swing.JButton;
import javax.swing.JFrame;
import javax.swing.JPanel;
import javax.swing.JTextArea;

public class MakePresets {
	JPanel preset, presetLabel, presetField;
	JTextArea namejta;
	JButton clear,save;
	
	
	JFrame frame = new JFrame();
	public MakePresets(){
		init();
	}

	private void init() {
		frame = new JFrame("Presets");
		frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
		frame.setLocationRelativeTo(null);
		frame.setVisible(true);
		frame.pack();
		frame.setSize(500, 500);
		
		preset = new JPanel(new GridLayout());
		presetLabel = new JPanel(new GridLayout());
		presetField = new JPanel(new GridLayout());
		
		String[] labels = {"Name: ", "How Many Numbers?"};
		String[] numbers = {"Number 1 Range: ",
				            "Number 2 Range: ",
				            "Number 3 Range: ",
				            "Number 4 Range: ",
				            "Number 5 Range: ",
				            "Number 6 Range: "};
		
		
		
		
		
		
		clear = new JButton("Clear");
		preset.add(clear);
		save = new JButton("Save");
		preset.add(save);
		
		
	}
	public static void main(String[] args){
		new MakePresets();
	}
}
/*
 * Name: 
 * Number of Numbers: 
 * Put the highest number that is allowed for each number
 * 
 * 
 * 
*/