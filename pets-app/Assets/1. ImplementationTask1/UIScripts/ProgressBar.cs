using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using UIWidgets;

public class ProgressBar : MonoBehaviour
{
    [SerializeField] private Slider progressSlider;
    public float fillSpeed = 0.5f;
    private float targetProgress = 0;
    public Text levelText;
    public static int currentLevel;
    public float timer = 0.0f; 
    private float minutes;
    private float seconds;

    private bool keepTiming = true;
    public GameObject notifyPad; 
    public Text notifyText;
    public float max; 


    private void Start()
    {
        IncrementFill(1f);
        currentLevel = 1;
        levelText.text = "Level " + currentLevel.ToString();
        Timer();
        max = 30; 
    }

    private void Update()
    {
        
        //int currentMinutes = (int)(timer / 60);
        //Debug.Log(currentMinutes);

        if (timer < max && keepTiming)
        {
            Timer();
            progressSlider.value = timer / max; 
        }
        else
        {
            ResetTimer();
            //LevelUp();
        }
    }

    public void IncrementFill(float newProgress)
    {
        targetProgress = progressSlider.value + newProgress;
    }

    public void LevelUp()
    {
    }

    public void Timer()
    {
        timer += Time.deltaTime;

    }

    public void ResetTimer()
    {
        progressSlider.value = 0f;
        timer = 0.0f;
        keepTiming = false;
        notifyPad.SetActive(true);
        int instantTLevelText = currentLevel + 1;
        notifyText.text = "You pet now is level " + instantTLevelText;
    }

    public void closePanel()
    {
        max *= 2;
        Debug.Log("max");
        Debug.Log(max);

        keepTiming = true;
        //UpdateSize.changeSize(currentLevel);

        notifyPad.SetActive(false);
        
        Debug.Log(levelText.text);
        currentLevel++;
        levelText.text = "Level " + currentLevel.ToString();
    }



}
