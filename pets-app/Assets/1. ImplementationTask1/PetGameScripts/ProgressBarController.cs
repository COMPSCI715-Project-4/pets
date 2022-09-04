using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI; 

public class ProgressBarController : MonoBehaviour
{
    public int min;
    public int max;
    public int current;
    public Image mask;
    public Text timeText;
    public float timeIncrese;
    public float maxTimeLevelUp = 1;
    private int level = 1;

    // Start is called before the first frame update
    void Start()
    {
        timeText.text = "Level" + level.ToString();
    }

    // Update is called once per frame
    void Update()
    {
        GetCurrentFill();
        //timeText.text = Time.deltaTime.ToString();
    }

    void GetCurrentFill()
    { 
        if (timeIncrese < maxTimeLevelUp)
        {
            timeIncrese += Time.deltaTime;
            mask.fillAmount = (float)timeIncrese / (float)maxTimeLevelUp;

        }
        else
        {
            level += 1;
            timeText.text = "Level" + level.ToString();
            Time.timeScale = 0;
            maxTimeLevelUp *= 2;
            Debug.Log(level);
            Debug.Log(maxTimeLevelUp);
            timeIncrese = 0;
        }

        

    }
}
