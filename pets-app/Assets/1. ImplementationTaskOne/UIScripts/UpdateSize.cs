using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI; 

public class UpdateSize : MonoBehaviour
{
    private GameObject pet;
    public Slider sizeSlider;
    // Start is called before the first frame update

    public void Start()
    {
        pet = GameObject.FindWithTag("pets");
    }

    private void Update()
    {
        changeSize();
    }
    public void changeSize()
    {
        
        if (pet != null)
        {
            Vector3 size = new Vector3(0.1f, 0.1f, 0.1f);
            pet.transform.localScale = sizeSlider.value * size;
            

        }
        else
        {
            pet = GameObject.FindWithTag("pets");

        }
    }
}
